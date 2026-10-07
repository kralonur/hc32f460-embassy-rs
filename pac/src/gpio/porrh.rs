#[doc = "Register `PORRH` reader"]
pub type R = crate::R<PorrhSpec>;
#[doc = "Register `PORRH` writer"]
pub type W = crate::W<PorrhSpec>;
#[doc = "Field `POR00` reader - desc POR00"]
pub type Por00R = crate::BitReader;
#[doc = "Field `POR00` writer - desc POR00"]
pub type Por00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR01` reader - desc POR01"]
pub type Por01R = crate::BitReader;
#[doc = "Field `POR01` writer - desc POR01"]
pub type Por01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR02` reader - desc POR02"]
pub type Por02R = crate::BitReader;
#[doc = "Field `POR02` writer - desc POR02"]
pub type Por02W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POR00"]
    #[inline(always)]
    pub fn por00(&self) -> Por00R {
        Por00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POR01"]
    #[inline(always)]
    pub fn por01(&self) -> Por01R {
        Por01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POR02"]
    #[inline(always)]
    pub fn por02(&self) -> Por02R {
        Por02R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POR00"]
    #[inline(always)]
    pub fn por00(&mut self) -> Por00W<'_, PorrhSpec> {
        Por00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POR01"]
    #[inline(always)]
    pub fn por01(&mut self) -> Por01W<'_, PorrhSpec> {
        Por01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POR02"]
    #[inline(always)]
    pub fn por02(&mut self) -> Por02W<'_, PorrhSpec> {
        Por02W::new(self, 2)
    }
}
#[doc = "desc PORRH\n\nYou can [`read`](crate::Reg::read) this register and get [`porrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PorrhSpec;
impl crate::RegisterSpec for PorrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`porrh::R`](R) reader structure"]
impl crate::Readable for PorrhSpec {}
#[doc = "`write(|w| ..)` method takes [`porrh::W`](W) writer structure"]
impl crate::Writable for PorrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORRH to value 0"]
impl crate::Resettable for PorrhSpec {}
