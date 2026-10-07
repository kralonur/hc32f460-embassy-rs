#[doc = "Register `PODRC` reader"]
pub type R = crate::R<PodrcSpec>;
#[doc = "Register `PODRC` writer"]
pub type W = crate::W<PodrcSpec>;
#[doc = "Field `POUT13` reader - desc POUT13"]
pub type Pout13R = crate::BitReader;
#[doc = "Field `POUT13` writer - desc POUT13"]
pub type Pout13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT14` reader - desc POUT14"]
pub type Pout14R = crate::BitReader;
#[doc = "Field `POUT14` writer - desc POUT14"]
pub type Pout14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT15` reader - desc POUT15"]
pub type Pout15R = crate::BitReader;
#[doc = "Field `POUT15` writer - desc POUT15"]
pub type Pout15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - desc POUT13"]
    #[inline(always)]
    pub fn pout13(&self) -> Pout13R {
        Pout13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POUT14"]
    #[inline(always)]
    pub fn pout14(&self) -> Pout14R {
        Pout14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POUT15"]
    #[inline(always)]
    pub fn pout15(&self) -> Pout15R {
        Pout15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc POUT13"]
    #[inline(always)]
    pub fn pout13(&mut self) -> Pout13W<'_, PodrcSpec> {
        Pout13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POUT14"]
    #[inline(always)]
    pub fn pout14(&mut self) -> Pout14W<'_, PodrcSpec> {
        Pout14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POUT15"]
    #[inline(always)]
    pub fn pout15(&mut self) -> Pout15W<'_, PodrcSpec> {
        Pout15W::new(self, 15)
    }
}
#[doc = "desc PODRC\n\nYou can [`read`](crate::Reg::read) this register and get [`podrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PodrcSpec;
impl crate::RegisterSpec for PodrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`podrc::R`](R) reader structure"]
impl crate::Readable for PodrcSpec {}
#[doc = "`write(|w| ..)` method takes [`podrc::W`](W) writer structure"]
impl crate::Writable for PodrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PODRC to value 0"]
impl crate::Resettable for PodrcSpec {}
