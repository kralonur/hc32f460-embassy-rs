#[doc = "Register `POSRH` reader"]
pub type R = crate::R<PosrhSpec>;
#[doc = "Register `POSRH` writer"]
pub type W = crate::W<PosrhSpec>;
#[doc = "Field `POS00` reader - desc POS00"]
pub type Pos00R = crate::BitReader;
#[doc = "Field `POS00` writer - desc POS00"]
pub type Pos00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS01` reader - desc POS01"]
pub type Pos01R = crate::BitReader;
#[doc = "Field `POS01` writer - desc POS01"]
pub type Pos01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS02` reader - desc POS02"]
pub type Pos02R = crate::BitReader;
#[doc = "Field `POS02` writer - desc POS02"]
pub type Pos02W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POS00"]
    #[inline(always)]
    pub fn pos00(&self) -> Pos00R {
        Pos00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POS01"]
    #[inline(always)]
    pub fn pos01(&self) -> Pos01R {
        Pos01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POS02"]
    #[inline(always)]
    pub fn pos02(&self) -> Pos02R {
        Pos02R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POS00"]
    #[inline(always)]
    pub fn pos00(&mut self) -> Pos00W<'_, PosrhSpec> {
        Pos00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POS01"]
    #[inline(always)]
    pub fn pos01(&mut self) -> Pos01W<'_, PosrhSpec> {
        Pos01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POS02"]
    #[inline(always)]
    pub fn pos02(&mut self) -> Pos02W<'_, PosrhSpec> {
        Pos02W::new(self, 2)
    }
}
#[doc = "desc POSRH\n\nYou can [`read`](crate::Reg::read) this register and get [`posrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PosrhSpec;
impl crate::RegisterSpec for PosrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`posrh::R`](R) reader structure"]
impl crate::Readable for PosrhSpec {}
#[doc = "`write(|w| ..)` method takes [`posrh::W`](W) writer structure"]
impl crate::Writable for PosrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POSRH to value 0"]
impl crate::Resettable for PosrhSpec {}
