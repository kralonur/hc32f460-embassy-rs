#[doc = "Register `POERH` reader"]
pub type R = crate::R<PoerhSpec>;
#[doc = "Register `POERH` writer"]
pub type W = crate::W<PoerhSpec>;
#[doc = "Field `POUTE00` reader - desc POUTE00"]
pub type Poute00R = crate::BitReader;
#[doc = "Field `POUTE00` writer - desc POUTE00"]
pub type Poute00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE01` reader - desc POUTE01"]
pub type Poute01R = crate::BitReader;
#[doc = "Field `POUTE01` writer - desc POUTE01"]
pub type Poute01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE02` reader - desc POUTE02"]
pub type Poute02R = crate::BitReader;
#[doc = "Field `POUTE02` writer - desc POUTE02"]
pub type Poute02W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POUTE00"]
    #[inline(always)]
    pub fn poute00(&self) -> Poute00R {
        Poute00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POUTE01"]
    #[inline(always)]
    pub fn poute01(&self) -> Poute01R {
        Poute01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POUTE02"]
    #[inline(always)]
    pub fn poute02(&self) -> Poute02R {
        Poute02R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POUTE00"]
    #[inline(always)]
    pub fn poute00(&mut self) -> Poute00W<'_, PoerhSpec> {
        Poute00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POUTE01"]
    #[inline(always)]
    pub fn poute01(&mut self) -> Poute01W<'_, PoerhSpec> {
        Poute01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POUTE02"]
    #[inline(always)]
    pub fn poute02(&mut self) -> Poute02W<'_, PoerhSpec> {
        Poute02W::new(self, 2)
    }
}
#[doc = "desc POERH\n\nYou can [`read`](crate::Reg::read) this register and get [`poerh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poerh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoerhSpec;
impl crate::RegisterSpec for PoerhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`poerh::R`](R) reader structure"]
impl crate::Readable for PoerhSpec {}
#[doc = "`write(|w| ..)` method takes [`poerh::W`](W) writer structure"]
impl crate::Writable for PoerhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POERH to value 0"]
impl crate::Resettable for PoerhSpec {}
